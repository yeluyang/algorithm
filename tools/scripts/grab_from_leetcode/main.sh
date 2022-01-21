#! /bin/bash

ROOT=$(dirname $0)

source $ROOT/profile

function log() {
	[ $VERBOSE ] && echo "[$(date)] $1";
	return 0;
}

function panic() {
	echo "[PANIC] $1";
	exit 1;
}

function format_single_file() {
	python3 $ROOT/fmt_lc_qd.py ${PYTHON_VERBOSE} $1 && ${MD_LINTERS} $1;
}

function format() {
	while [ $# -gt 0 ]; do
		ls $1.*/README.md > /dev/null || shift && continue;
		DESC_FILE=$(ls $1.*/README.md);
		if [ ${#DESC_FILE[*]} -gt 1 ]
		then
			panic "found ${#DESC_FILE[*]} files for $1: ${DESC_FILE}";
		elif [ ${#DESC_FILE[*]} -eq 1 ]
		then
			log "found ${#DESC_FILE[*]} files for $1: ${DESC_FILE}";
			format_single_file ${DESC_FILE[0]};
		fi

		shift;
	done
}

function grab_raw() {
	log "grab question $1 with [$2] code"
	SLEEP_TIME=$(($RANDOM%5+5));
	${LEETCODE_BIN_DIR}/leetcode show $1 -gx -l $2 -o . > $1.md && \
		log "sleep ${SLEEP_TIME}s after grab question $1" && sleep ${SLEEP_TIME};
}

function grab() {
	while [ $# -gt 0 ]; do
		log "grab question $1 from leetcode"

		for i in ${!CODE_LANGS[*]};
		do
			grab_raw $1 ${CODE_LANGS[$i]}
			ls $1.*.${CODE_EXTS[$i]} > /dev/null || continue;
			CODE_FILE=$(ls $1.*.${CODE_EXTS[$i]});
			if [ ${#CODE_FILE[*]} -gt 1 ]
			then
				panic "found ${#CODE_FILE[*]} files for $1: ${CODE_FILE}";
			elif [ ${#CODE_FILE[*]} -eq 1 ]
			then
				log "found ${#CODE_FILE[*]} files for $1: ${CODE_FILE}";
				DIR_NAME=$(echo ${CODE_FILE[0]} | sed "s/\.${CODE_EXTS[$i]}$//");
				log "package $DIR_NAME as dir with ${CODE_FILE[0]} and $1.md";
				mkdir $DIR_NAME && mv ${CODE_FILE[0]} $DIR_NAME && mv $1.md $DIR_NAME/README.md \
					&& format_single_file $DIR_NAME/README.md;

				break 1;
			fi
		done

		shift;
	done
}

function help() {
	echo "main.sh <grab | format | grab_raw | format_single_file> <QUESTION_ID[, QUESTION_ID, ...]>"
	echo ""
	echo "command"
	echo "	grab: grab questions from leetcode then organize and format it by id"
	echo "	format: format leetcode questions locally by id"
	echo "	grab_raw: grab questions from leetcode without organize or format it by id"
	echo "	format_single_file: format leetcode questions locally by local path of questions"
	echo ""
	echo "arguments"
	echo "	QUESTIONS_ID: id of questions from leetcode. can give more than one"
}

[ $# -le 0 ] && help && panic "command is required"
COMMAND=$1
shift

[ $# -le 0 ] && help && panic "required one id of question at least"
$COMMAND $*

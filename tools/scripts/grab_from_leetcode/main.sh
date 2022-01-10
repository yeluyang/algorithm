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
	log "formatting $1:";

	log "fixing title";
	sed -i '1s/^\[\([0-9]\+\)\] /# \1\./' $1;
	sed -i '3s_^\(https://leetcode-cn.com/problems/.*/description/\)$_<\1>_' $1;

	log "removing useless";
	sed -i '5s/^Tags: .*$//' $1;
	sed -i '7s/^Langs: .*$//' $1;
	sed -i '9s/^* algorithms$//' $1;
	sed -i '10s/^* \(Easy\|Medium\|Hard\) ([0-9].*\.[0-9].*%)$//' $1;
	sed -i '11s/^* Likes: .*$//' $1;
	sed -i '12s/^* Dislikes: .*$//' $1;
	sed -i '13s/^* Total Accepted: .*$//' $1;
	sed -i '14s/^* Total Submissions: .*$//' $1;
	sed -i '15s/^* Testcase Example: .*$//' $1;
	sed -i '16s/^* Source Code: .*$//' $1;
	sed -i 's/&nbsp;//g' $1;

	log "removing <br>";
	sed -i 's/<br \?\/\?>/\n/g' $1;

	log "removing <p>";
	sed -i 's/<\/\?p>//g' $1;

	log "removing <div>";
	sed -i 's/<\/\?div[^<>]*>//g' $1;

	log "removing <small>";
	sed -i 's/<\/\?small>//g' $1;

	log "removing <strong>";
	sed -i 's/<\/\?strong>//g' $1;

	log "removing <b>";
	sed -i 's/<\/\?b>//g' $1;

	log "removing <em>";
	sed -i 's/<\/\?em>//g' $1;

	log "formatting <pre>";
	sed -i 's/^\(.\+\)<pre>/\1\n<pre>/g' $1;
	sed -i 's/^\(.\+\)<\/pre>/\1\n<\/pre>/g' $1;
	sed -i 's/<pre>\(.\+\)$/<pre>\n\1/g' $1;
	sed -i 's/<\/pre>\(.\+\)$/<\/pre>\n\1/g' $1;

	log "parsing <pre>";
	PRE_STARTS=($(sed -n '/<pre>/=' $1 | xargs));
	PRE_ENDS=($(sed -n '/<\/pre>/=' $1 | xargs));
	log "parsing <pre> has start=[${PRE_STARTS[@]}], end=[${PRE_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#PRE_STARTS[@]} - 1) ); do
		START=${PRE_STARTS[i]};
		END=${PRE_ENDS[i]};
		log "parsing <code> from <pre:$i>[$START:$END]";
		sed -i "${START},${END}s/<\/\?code>//g" $1;
	done
	sed -i 's/<pre>/```txt/g' $1;
	sed -i 's/<\/pre>/```/g' $1;

	log "formatting <ul>";
	sed -i 's/^\(.\+\)<ul>/\1\n<ul>/g' $1;
	sed -i 's/^\(.\+\)<\/ul>/\1\n<\/ul>/g' $1;
	sed -i 's/<ul>\(.\+\)$/<ul>\n\1/g' $1;
	sed -i 's/<\/ul>\(.\+\)$/<\/ul>\n\1/g' $1;

	log "formatting <ol>";
	sed -i 's/^\(.\+\)<ol>/\1\n<ol>/g' $1;
	sed -i 's/^\(.\+\)<\/ol>/\1\n<\/ol>/g' $1;
	sed -i 's/<ol>\(.\+\)$/<ol>\n\1/g' $1;
	sed -i 's/<\/ol>\(.\+\)$/<\/ol>\n\1/g' $1;

	log "formatting <li>";
	sed -i 's/\(\s\|\xC2\xA0\)\+<li>/<li>/g' $1;
	sed -i 's/^\(.\+\)<li>/\1\n<li>/g' $1;
	sed -i 's/<\/li>\(.\+\)$/<\/li>\n\1/g' $1;

	log "removing empty <li>";
	sed -i 's/<li>\(\s\|\xC2\xA0\)\?<\/li>//g' $1;

	LI_STARTS=($(sed -n '/<ul>/=' $1 | xargs));
	LI_ENDS=($(sed -n '/<\/ul>/=' $1 | xargs));
	log "parsing <ul> has start=[${LI_STARTS[@]}], end=[${LI_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#LI_STARTS[@]} - 1) ); do
		START=${LI_STARTS[i]};
		END=${LI_ENDS[i]};
		log "parsing <li> in <ul:$i>[$START:$END]";
		sed -i "${START},${END}s/^<li>\(\s\|\xC2\xA0\)\?/- /" $1;
		sed -i "${START},${END}s/\(\s\|\xC2\xA0\)\?<\/li>//" $1;
	done
	sed -i 's/<ul>//' $1;
	sed -i 's/<\/ul>//' $1;

	LI_STARTS=($(sed -n '/<ol>/=' $1 | xargs));
	LI_ENDS=($(sed -n '/<\/ol>/=' $1 | xargs));
	log "parsing <ol> has start=[${LI_STARTS[@]}], end=[${LI_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#LI_STARTS[@]} - 1) ); do
		START=${LI_STARTS[i]};
		END=${LI_ENDS[i]};
		log "parsing <li> in <ol:$i>[$START:$END]";
		sed -i "${START},${END}s/^<li>\(\s\|\xC2\xA0\)\?/1\. /" $1;
		sed -i "${START},${END}s/\(\s\|\xC2\xA0\)\?<\/li>//" $1;
	done
	sed -i 's/<ol>//' $1;
	sed -i 's/<\/ol>//' $1;

	log "removing empty <code>";
	sed -i 's/<code>\(\s\|\xC2\xA0\)\?<\/code>//g' $1;

	log "parsing <code>";
	sed -i 's/<\/\?code>/`/g' $1;

	log "compressing <img> nested in <a href>";
	sed -i 's/<a href=[^<>]\?>\(<img [^<>]*\/>\)<\/a>/\1/g' $1;

	log "formatting <img>";
	sed -i 's/^\(.\+\)\(<img [^<>]*\/>\)/\1\n\2/g' $1;
	sed -i 's/\(<img [^<>]*\/>\)\(.\+\)$/<img [^<>]*\/>\1\n\2/g' $1;

	IMG_LINES=$(sed -n '/^<img [^<>]*\/\?>$/=' $1 | xargs);
	log "parsing <img>: [${IMG_LINES}]";
	for i in $IMG_LINES; do
		LINE=$(sed -n "${i}p" $1);
		ALT=$(echo $LINE | awk -F 'alt=' '{print $2}' | awk -F '"' '{print $2}');
		SRC=$(echo $LINE | awk -F 'src=' '{print $2}' | awk -F '"' '{print $2}');
		NAME=${ALT:-$(basename $SRC)};
		log "parsing <img>-$i=$LINE, get name=$NAME, src=$SRC";
		sed -i "${i}s|^<img [^<>]*/\?>$|![$NAME]($SRC)|" $1;
	done

	log "removing spaces in <a href>";
	sed -i 's/\(<a href=[^<>]*>\)\(\s\|\xC2\xA0\)\+/\1/g' $1;
	sed -i 's/\(\s\|\xC2\xA0\)\+<\/a>/<\/a>/g' $1;

	log "parsing <a href> after all <img> have been handled";
	sed -i 's/<a href="\([^"<>]*\)"[^<>]*>\([^<>]*\)<\/a>/[\2](\1)/g' $1;

	log "parsing <sup>";
	perl -pe 's/<sup>(.*?)<\/sup>/\^\1/g' -i $1;

	log "parsing O(xx)";
	perl -pe 's/(^|[^\$])(O\(.*?\))($|[^\$])/\$\2\$/g' -i $1;

	log "parsing lt, gt, le, ge";
	sed -i 's/&lt;/</g' $1;
	sed -i 's/&gt;/>/g' $1;
	sed -i 's/&le;/<=/g' $1;
	sed -i 's/&ge;/>=/g' $1;

	log "removing trailing spaces";
	sed -i 's/\(\s\|\xC2\xA0\)\+$//' $1;

	log "removing useless empty lines";
	sed -i '/^$/N;/^\n$/D' $1;

	# now you should use tools to lint&format markdown manually
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

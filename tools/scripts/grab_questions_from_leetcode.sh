#! /bin/bash

for n in $QIDS; do
	${HOME}/.vscode-server/extensions/leetcode.vscode-leetcode-0.18.0/node_modules/vsc-leetcode-cli/bin/leetcode show $n -gx -l rust -o . > $n.md;
	RAND_SLEEP=$(($RANDOM%5+5));
	echo "sleeping $RAND_SLEEP s after $n";
	sleep $RAND_SLEEP;
done

for n in $(ls *.rs | xargs); do
	DIR_NAME=$(echo $n | sed "s/\.rs//g");
	Q_NUM=$(echo $DIR_NAME | awk -F '.' '{print $1}');
	echo "moving $Q_NUM.md and $n into $DIR_NAME";
	mkdir $DIR_NAME;
	mv $n $DIR_NAME;
	mv $Q_NUM.md $DIR_NAME/README.md;
done

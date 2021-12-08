#! /bin/bash

for f in $QFILES; do

	echo "processing $f:";

	echo "fixing title";
	sed -i '1s/^\[\([0-9]\+\)\] /# \1\./' $f;
	sed -i '3s_^\(https://leetcode-cn.com/problems/.*/description/\)$_<\1>_' $f;

	echo "removing useless";
	sed -i '5s/^Tags: .*$//' $f;
	sed -i '7s/^Langs: .*$//' $f;
	sed -i '9s/^* algorithms$//' $f;
	sed -i '10s/^* \(Easy\|Medium\|Hard\) ([0-9].*\.[0-9].*%)$//' $f;
	sed -i '11s/^* Likes: .*$//' $f;
	sed -i '12s/^* Dislikes: .*$//' $f;
	sed -i '13s/^* Total Accepted: .*$//' $f;
	sed -i '14s/^* Total Submissions: .*$//' $f;
	sed -i '15s/^* Testcase Example: .*$//' $f;
	sed -i '16s/^* Source Code: .*$//' $f;
	sed -i 's/&nbsp;//g' $f;

	echo "removing <br>";
	sed -i 's/<br \?\/\?>/\n/g' $f;

	echo "removing <p>";
	sed -i 's/<\/\?p>//g' $f;

	echo "removing <div>";
	sed -i 's/<\/\?div[^<>]*>//g' $f;

	echo "removing <small>";
	sed -i 's/<\/\?small>//g' $f;

	echo "removing <strong>";
	sed -i 's/<\/\?strong>//g' $f;

	echo "removing <b>";
	sed -i 's/<\/\?b>//g' $f;

	echo "removing <em>";
	sed -i 's/<\/\?em>//g' $f;

	echo "formatting <pre>";
	sed -i 's/^\(.\+\)<pre>/\1\n<pre>/g' $f;
	sed -i 's/^\(.\+\)<\/pre>/\1\n<\/pre>/g' $f;
	sed -i 's/<pre>\(.\+\)$/<pre>\n\1/g' $f;
	sed -i 's/<\/pre>\(.\+\)$/<\/pre>\n\1/g' $f;

	echo "parsing <pre>";
	PRE_STARTS=($(sed -n '/<pre>/=' $f | xargs));
	PRE_ENDS=($(sed -n '/<\/pre>/=' $f | xargs));
	echo "parsing <pre> has start=[${PRE_STARTS[@]}], end=[${PRE_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#PRE_STARTS[@]} - 1) ); do
		START=${PRE_STARTS[i]};
		END=${PRE_ENDS[i]};
		echo "parsing <code> from <pre:$i>[$START:$END]";
		sed -i "${START},${END}s/<\/\?code>//g" $f;
	done
	sed -i 's/<pre>/```txt/g' $f;
	sed -i 's/<\/pre>/```/g' $f;

	echo "formatting <ul>";
	sed -i 's/^\(.\+\)<ul>/\1\n<ul>/g' $f;
	sed -i 's/^\(.\+\)<\/ul>/\1\n<\/ul>/g' $f;
	sed -i 's/<ul>\(.\+\)$/<ul>\n\1/g' $f;
	sed -i 's/<\/ul>\(.\+\)$/<\/ul>\n\1/g' $f;

	echo "formatting <ol>";
	sed -i 's/^\(.\+\)<ol>/\1\n<ol>/g' $f;
	sed -i 's/^\(.\+\)<\/ol>/\1\n<\/ol>/g' $f;
	sed -i 's/<ol>\(.\+\)$/<ol>\n\1/g' $f;
	sed -i 's/<\/ol>\(.\+\)$/<\/ol>\n\1/g' $f;

	echo "formatting <li>";
	sed -i 's/\(\s\|\xC2\xA0\)\+<li>/<li>/g' $f;
	sed -i 's/^\(.\+\)<li>/\1\n<li>/g' $f;
	sed -i 's/<\/li>\(.\+\)$/<\/li>\n\1/g' $f;

	echo "removing empty <li>";
	sed -i 's/<li>\(\s\|\xC2\xA0\)\?<\/li>//g' $f;

	LI_STARTS=($(sed -n '/<ul>/=' $f | xargs));
	LI_ENDS=($(sed -n '/<\/ul>/=' $f | xargs));
	echo "parsing <ul> has start=[${LI_STARTS[@]}], end=[${LI_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#LI_STARTS[@]} - 1) ); do
		START=${LI_STARTS[i]};
		END=${LI_ENDS[i]};
		echo "parsing <li> in <ul:$i>[$START:$END]";
		sed -i "${START},${END}s/^<li>\(\s\|\xC2\xA0\)\?/- /" $f;
		sed -i "${START},${END}s/\(\s\|\xC2\xA0\)\?<\/li>//" $f;
	done
	sed -i 's/<ul>//' $f;
	sed -i 's/<\/ul>//' $f;

	LI_STARTS=($(sed -n '/<ol>/=' $f | xargs));
	LI_ENDS=($(sed -n '/<\/ol>/=' $f | xargs));
	echo "parsing <ol> has start=[${LI_STARTS[@]}], end=[${LI_ENDS[@]}]";
	for i in $(seq 0 $(expr ${#LI_STARTS[@]} - 1) ); do
		START=${LI_STARTS[i]};
		END=${LI_ENDS[i]};
		echo "parsing <li> in <ol:$i>[$START:$END]";
		sed -i "${START},${END}s/^<li>\(\s\|\xC2\xA0\)\?/1\. /" $f;
		sed -i "${START},${END}s/\(\s\|\xC2\xA0\)\?<\/li>//" $f;
	done
	sed -i 's/<ol>//' $f;
	sed -i 's/<\/ol>//' $f;

	echo "removing empty <code>";
	sed -i 's/<code>\(\s\|\xC2\xA0\)\?<\/code>//g' $f;

	echo "parsing <code>";
	sed -i 's/<\/\?code>/`/g' $f;

	echo "compressing <img> nested in <a href>";
	sed -i 's/<a href=[^<>]\?>\(<img [^<>]*\/>\)<\/a>/\1/g' $f;

	echo "formatting <img>";
	sed -i 's/^\(.\+\)\(<img [^<>]*\/>\)/\1\n\2/g' $f;
	sed -i 's/\(<img [^<>]*\/>\)\(.\+\)$/<img [^<>]*\/>\1\n\2/g' $f;

	IMG_LINES=$(sed -n '/^<img [^<>]*\/>$/=' $f | xargs);
	echo "parsing <img>: [${IMG_LINES}]";
	for i in $IMG_LINES; do
		LINE=$(sed -n "${i}p" $f);
		ALT=$(echo $LINE | awk -F 'alt=' '{print $2}' | awk -F '"' '{print $2}');
		SRC=$(echo $LINE | awk -F 'src=' '{print $2}' | awk -F '"' '{print $2}');
		NAME=${ALT:-$(basename $SRC)};
		echo "parsing <img>-$i=$LINE, get name=$NAME, src=$SRC";
		sed -i "${i}s|^<img [^<>]*/>$|![$NAME]($SRC)|" $f;
	done

	echo "removing spaces in <a href>";
	sed -i 's/\(<a href=[^<>]*>\)\(\s\|\xC2\xA0\)\+/\1/g' $f;
	sed -i 's/\(\s\|\xC2\xA0\)\+<\/a>/<\/a>/g' $f;

	echo "parsing <a href> after all <img> have been handled";
	sed -i 's/<a href="\([^"<>]*\)"[^<>]*>\([^<>]*\)<\/a>/[\2](\1)/g' $f;

	echo "parsing <sup>";
	perl -pe 's/<sup>(.*?)<\/sup>/\^\1/g' -i $f;

	echo "parsing O(xx)";
	perl -pe 's/(^|[^\$])(O\(.*?\))($|[^\$])/\$\2\$/g' -i $f;

	echo "parsing lt, gt, le, ge";
	sed -i 's/&lt;/</g' $f;
	sed -i 's/&gt;/>/g' $f;
	sed -i 's/&le;/<=/g' $f;
	sed -i 's/&ge;/>=/g' $f;

	echo "removing trailing spaces";
	sed -i 's/\(\s\|\xC2\xA0\)\+$//' $f;

	echo "removing useless empty lines";
	sed -i '/^$/N;/^\n$/D' $f;

	# now you should use tools to lint&format markdown manually

done

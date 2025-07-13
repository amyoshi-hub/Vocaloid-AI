for f in *_*; do
	newname=$(echo "$f" | sed -E 's/^＿+//; s/＿+/_/g; s/__+/_/; s/__+/_/; s/＿wav/\.wav/')
	echo "$f -> $newname"
done

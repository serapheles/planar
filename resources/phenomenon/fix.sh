for file in *.png
do
  mv -- "$file" "${file/\"/}"
done

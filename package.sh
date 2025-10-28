if [[ -z "$1" ]]; then
	echo "Provide cmdcreate's version (MUST NOT START WITH v)"
	exit 1
fi

./format.sh

cd ./package

echo creating bin....
./create_bin.sh $1
echo creating deb....
./create_deb.sh $1
echo creating rpm....
./create_rpm.sh $1

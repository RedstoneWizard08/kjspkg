if [ `id -u` != 0 ]
then
    sudo -v
fi

echo "Installation started..."

sudo rm -f /usr/local/bin/kjspkg

sudo sh -c "curl -s https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/main/app.py > /usr/local/bin/kjspkg"
python3 -m pip -q install $(curl -s https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/main/requirements.txt) > /dev/null
sudo chmod +x /usr/local/bin/kjspkg

echo "Done!"
#!/bin/bash

sudo apt-get install mysql-server
sudo apt-get install mysql-client

read -p "new user: " user
read -p "new user password: " password

sudo mysql -u root -p << EOF
CREATE USER '$user'@'localhost' IDENTIFIED BY '$password';
GRANT ALL PRIVILEGES ON *.* TO '$user'@'localhost';
FLUSH PRIVILEGES;
SELECT user FROM mysql.user;
EOF

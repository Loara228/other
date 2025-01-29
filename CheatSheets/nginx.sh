
#!/bin/bash

sudo apt install nginx

rm /etc/nginx/sites-available/default
cd /etc/nginx/sites-available

read -p "domain: " domain

cat << EOF > default
server {
    listen        80;
    server_name $domain;
    location / {
        proxy_pass         http://127.0.0.1:5000;
        proxy_http_version 1.1;
        proxy_set_header   Upgrade \$http_upgrade;
        proxy_set_header   Connection keep-alive;
        proxy_set_header   Host \$host;
        proxy_cache_bypass \$http_upgrade;
        proxy_set_header   X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto \$scheme;
    }
}
EOF
cat default

sudo nginx -s reload

oha "http://localhost:3000/greet" -H "cookie: $(just get_auth_cookie)" -z 5s -c 150 --disable-compression

-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES ('d84c242f-2435-42a1-82ad-8b13dd633b75',
        'admin',
        '$argon2id$v=19$m=19456,t=2,p=1$WRe+cwSBOftNsUQWu6DlyQ$g88saZTgyB1exEb6iUmX3Iu766/RUPVLYoqDNR6l5y0');
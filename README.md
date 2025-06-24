# ハッカソン デモアプリケーション

# 開発方法
## Dcokerコマンド
プロジェクトルート : `docker compose up --build`
## 注意


# 参考
https://chatgpt.com/c/6853c13b-1114-8004-b659-1216d6b319df
axum: https://chatgpt.com/share/6856c4b5-c210-8004-beb9-36b1c231121c

# Lightsail
```
#!/bin/bash
# UbuntuにDockerをインストール
apt-get update
apt-get install -y apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
apt-get update
apt-get install -y docker-ce
usermod -a -G docker ubuntu

# Docker Composeをインストール
curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Gitをインストール
apt-get install -y git
```

# 次回
- バックエンドを外部から叩けないようにする:https://gemini.google.com/app/abe9305d39109cb3
- httpsの設定
- 独自ドメインの設定

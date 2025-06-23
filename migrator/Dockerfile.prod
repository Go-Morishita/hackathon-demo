FROM rust:latest

WORKDIR /app

# SQLx CLI + MySQL クライアント
RUN cargo install sqlx-cli --no-default-features --features mysql \
 && apt-get update \
 && apt-get install -y default-mysql-client \
 && rm -rf /var/lib/apt/lists/*

# スクリプトと設定／マイグレーションをコピー
COPY wait-for-mysql.sh .
COPY sqlx.toml .
COPY migrations ./migrations

# 権限を付与
RUN chmod +x wait-for-mysql.sh

# スクリプトをエントリポイントにしてマイグレーション
CMD ["./wait-for-mysql.sh", "db", "3306", "hackathon_demo_db", "hackathon_demo_admin", "hackathon_demo", "sqlx migrate run"]
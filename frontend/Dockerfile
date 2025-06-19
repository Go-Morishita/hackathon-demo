FROM node:18-alpine

WORKDIR /app

COPY next/package*.json ./
RUN npm install

COPY next/ ./

EXPOSE 3000
CMD ["npm", "run", "dev"]

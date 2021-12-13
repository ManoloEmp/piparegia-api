FROM node:12.18.1-alpine

ENV NODE_ENV=production

WORKDIR /app

COPY ["package.json", "package-lock.json*", "./"]

COPY prisma ./prisma/

RUN npm install

COPY . .

CMD ["npm", "start"]

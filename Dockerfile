FROM node:12.18.1-alpine

ENV NODE_ENV=production

WORKDIR /app

COPY ["package.json", "package-lock.json*", "./"]

RUN npm install

COPY . .

EXPOSE 4000

RUN cd src/prisma  && npx prisma generate


CMD ["npm", "start"]

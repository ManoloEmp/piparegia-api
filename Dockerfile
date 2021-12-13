FROM node:12.18.1-alpine

ENV NODE_ENV=production

# Create app directory
WORKDIR /app

# A wildcard is used to ensure both package.json AND package-lock.json are copied
COPY package*.json ./
COPY src/prisma ./src/prisma/

# Install app dependencies
RUN npm install

RUN npx prisma generate

COPY . .

CMD ["npm", "start"]

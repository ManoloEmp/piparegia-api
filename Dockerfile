FROM node:12.18.1-alpine

# Create app directory
WORKDIR /app



# A wildcard is used to ensure both package.json AND package-lock.json are copied
COPY package*.json ./
COPY ./src /app/src/

# Install app dependencies
RUN npm install

# RUN npm i -g prisma@3.6.0

RUN cd src/ && npx prisma generate

EXPOSE 4000

CMD ["npm", "start"]

FROM node:16.2-alpine as build

WORKDIR /recipebook

COPY . .

RUN npm install
RUN npm run build

ENTRYPOINT [ "node", "build" ]


FROM node:25-alpine3.22 AS build

COPY package.json /frontend/
COPY package-lock.json /frontend/

WORKDIR /frontend

RUN npm install

COPY . /frontend/

RUN npm run build


FROM nginx:latest

COPY nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=build /frontend/dist /usr/share/nginx/html

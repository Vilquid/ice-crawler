# premier docker
FROM node:19.9.0 as build


WORKDIR /usr/local/app


COPY ./ /usr/local/app/


RUN npm install


RUN npm run build


# deuxième docker


FROM nginx:latest


COPY --from=build /usr/local/app/dist/icecrawlerui /usr/share/nginx/html

EXPOSE 80

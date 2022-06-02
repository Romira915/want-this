FROM node:16.15.0 AS develop
WORKDIR /want-this
RUN yarn add --dev tailwindcss
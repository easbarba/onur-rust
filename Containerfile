# Onur is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Onur is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Onur. If not, see <https://www.gnu.org/licenses/>.

FROM rust:1.80

ENV USERNAME easbarba
ENV APP_HOME /home/$USERNAME/app
WORKDIR $APP_HOME

RUN groupadd -r $USERNAME && useradd -r -g $USERNAME -d /home/$USERNAME -m -s /bin/bash $USERNAME
RUN chown -R $USERNAME:$USERNAME /home/$USERNAME

COPY examples examples
COPY ./prepare.bash .
RUN ./prepare.bash

COPY Cargo.toml Cargo.lock .
RUN mkdir src; echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm ./src/main.rs

COPY . .
RUN cargo build

CMD [ "cargo", "test" ]

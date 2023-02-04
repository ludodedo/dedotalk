import pprint
from dataclasses import dataclass


@dataclass
class User:
    id: int
    email: str


pp = pprint.PrettyPrinter(indent=4)
user_1 = User(1, "alice@dedomainia.com")
user_2 = User(2, "bob@dedomainia.com")
user_3 = User(3, "charlie@gmail.com")

all_user_list = [user_1, user_2, user_3]
user_with_dedo_mail = [
    user for user in all_user_list if user.email.endswith("dedomainia.com")
]

pp.pprint(user_with_dedo_mail)

import argparse
from near_api.providers import JsonProvider
from near_api.signer import KeyPair, Signer
from near_api.account import Account

parser = argparse.ArgumentParser(description='sbt-marketplace-oracle')
parser.add_argument('account_id', type=str, help='user account id')
parser.add_argument('private_key', type=str, help='sum the integers (default: find the max)')

oracle_contract_id = "dev-1663022799979-96778451185412"

def create_account(private_key, account_id):
    near_provider = JsonProvider("https://rpc.testnet.near.org")

    sender_key_pair = KeyPair(private_key)
    sender_signer = Signer(account_id, sender_key_pair)
    print(sender_signer, account_id)
    return Account(near_provider, sender_signer, account_id)

def get_next_request(account: Account):
    return account.view_function(oracle_contract_id, "get_next_request", {})['result']

def main():
    args = parser.parse_args()
    account = create_account(args.private_key, args.account_id)

    while (request := get_next_request(account)):
        print(request)
    print("Done")

if __name__ == "__main__":
    main()

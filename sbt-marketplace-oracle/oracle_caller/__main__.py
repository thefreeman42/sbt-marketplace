import argparse
from near_api.providers import JsonProvider
from near_api.signer import KeyPair, Signer
from near_api.account import Account
import requests

parser = argparse.ArgumentParser(description='sbt-marketplace-oracle')
parser.add_argument('account_id', type=str, help='user account id')
parser.add_argument('private_key', type=str, help='sum the integers (default: find the max)')

oracle_contract_id = "dev-1663022799979-96778451185412"
endpoint = "https://rpc.testnet.near.org"

def create_account(private_key, account_id):
    near_provider = JsonProvider(endpoint)

    sender_key_pair = KeyPair(private_key)
    sender_signer = Signer(account_id, sender_key_pair)
    return Account(near_provider, sender_signer, account_id)

def get_next_request(account: Account):
    return account.view_function(oracle_contract_id, "get_next_request", {})['result']

def get_account_keys(account_id: str):
    result = requests.post(
        endpoint,
        json={
            "jsonrpc": "2.0",
            "id": "myid",
            "method": "query",
            "params": {
                "request_type": "view_access_key_list",
                "finality": "final",
                "account_id": account_id
            }
        })
    return result.json()["result"]["keys"]

def submit_oracle(account, account_id, public_key, result):
    return account.function_call(
        oracle_contract_id, "apply_next_request", {
            "action": [account_id, public_key],
            "result": result
        },
        gas=300000000000000)


def main():
    args = parser.parse_args()
    account = create_account(args.private_key, args.account_id)

    while (request := get_next_request(account)):
        account_id, public_key = request
        print(account_id, public_key)
        keys = get_account_keys(account_id)
        exists = any(key["public_key"] == public_key for key in keys)
        print(submit_oracle(account, account_id, public_key, exists))
    print("Done")

if __name__ == "__main__":
    main()

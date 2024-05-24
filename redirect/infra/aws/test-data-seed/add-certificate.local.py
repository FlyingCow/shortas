import sys
import json
import logging
import os
import boto3
from botocore.exceptions import ClientError
AWS_REGION = 'us-east-1'
AWS_PROFILE = 'default'
LOCALSTACK_ENDPOINT_URL = os.environ.get('LOCALSTACK_ENDPOINT_URL', 'http://localhost:4566')
# logger config
logger = logging.getLogger()
logging.basicConfig(level=logging.INFO,
                    format='%(asctime)s: %(levelname)s: %(message)s')
boto3.setup_default_session()
dynamodb_resource = boto3.resource(
    "dynamodb", region_name=AWS_REGION, endpoint_url=LOCALSTACK_ENDPOINT_URL, )

def add_dynamodb_table_item(table_name, hostname, cert, key):
    """
    adds a DynamoDB table.
    """
    try:
        table = dynamodb_resource.Table(table_name)
        response = table.put_item(
            Item={
                'hostname': hostname,
                'cert': cert,
                'key': key
            }
        )
    except ClientError:
        logger.exception('Could not add the item to table.')
        raise
    else:
        return response

def main():
    """
    Main invocation function.
    """
    table_name = 'core-routes-encryption-local'
    
    hostname_param = sys.argv[1]
    cert_param = sys.argv[2]
    key_param = sys.argv[3]

    hostname = hostname_param
    cert = ''
    key = ''


    with open(cert_param) as f:
        cert = f.read()

    with open(key_param) as f:
        key = f.read()

    logger.info('Adding item...')
    dynamodb = add_dynamodb_table_item(table_name, hostname, cert, key)

    logger.info(
        f'DynamoDB table item created: {json.dumps(dynamodb, indent=4)}')

if __name__ == '__main__':
    main()

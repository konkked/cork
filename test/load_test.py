import argparse
import asyncio
import aiohttp
import random
import string
import time

BASE_URL = 'http://127.0.0.1:3030'
KEY_PREFIX = 'key_'
VALUE_PREFIX = 'value_'

def generate_random_string(length=10):
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

async def post_set(session, key, value):
    async with session.post(f'{BASE_URL}/set', json={'key': key, 'value': value}) as response:
        return response

async def get(session, key):
    async with session.get(f'{BASE_URL}/get', params={'key': key}) as response:
        return response

async def delete(session, key):
    async with session.delete(f'{BASE_URL}/remove', params={'key': key}) as response:
        return response

async def perform_request(session, key, value):
    # Set operation
    start = time.time()
    response = await post_set(session, key, value)
    set_time = time.time() - start

    # Get operation
    start = time.time()
    response = await get(session, key)
    get_time = time.time() - start

    # Delete operation
    start = time.time()
    response = await delete(session, key)
    delete_time = time.time() - start

    return set_time, get_time, delete_time, response.status

async def main(num_requests):
    start_time = time.time()
    
    async with aiohttp.ClientSession() as session:
        tasks = []
        set_times = []
        get_times = []
        delete_times = []
        successes = 0
        errors = 0

        for _ in range(num_requests):
            key = f'{KEY_PREFIX}{generate_random_string()}'
            value = f'{VALUE_PREFIX}{generate_random_string()}'
            tasks.append(perform_request(session, key, value))

        results = await asyncio.gather(*tasks)

        for set_time, get_time, delete_time, status in results:
            set_times.append(set_time)
            get_times.append(get_time)
            delete_times.append(delete_time)

            if status == 200:
                successes += 1
            else:
                errors += 1

    end_time = time.time()
    total_time = end_time - start_time
    requests_per_second = num_requests / total_time

    print(f'Number of Requests: {num_requests}')
    print(f'Average Set Time: {sum(set_times) / len(set_times):.4f} seconds')
    print(f'Average Get Time: {sum(get_times) / len(get_times):.4f} seconds')
    print(f'Average Delete Time: {sum(delete_times) / len(delete_times):.4f} seconds')
    print(f'Total Successes: {successes}')
    print(f'Total Errors: {errors}')
    print(f'Total Time: {total_time:.2f} seconds')
    print(f'Requests Per Second: {requests_per_second:.2f}')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Load test the key-value store.')
    parser.add_argument('num_requests', type=int, help='Number of requests to perform')
    args = parser.parse_args()
    asyncio.run(main(args.num_requests))

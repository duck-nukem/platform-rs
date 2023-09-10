# pip install locust
from locust import HttpUser, between, task


# locust -f perf_test.py
class WebsiteUser(HttpUser):
    host = "http://localhost:3000/"
    wait_time = between(5, 15)

    def on_start(self):
        self.client.post(
            "login", {
                "username": "admin",
                "password": "pass"
            },
            headers={
                'Content-Type': 'application/x-www-form-urlencoded',
            },
        )

    @task
    def index(self):
        self.client.get("greet")

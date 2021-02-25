import time
from locust import HttpUser, task, between

class Basic(HttpUser):
    wait_time = between(0.5, 0.5)

    @task
    def read(self):
        self.client.get("/cpu")

    # @task
    # def read(self):
    #     self.client.get("/cpualloc")

from locust import HttpUser, task, between

class Basic(HttpUser):
    wait_time = between(0.5, 0.5)

    # @task
    # def read(self):
    #     self.client.get("/read")

    @task
    def read(self):
        self.client.get("/fast")

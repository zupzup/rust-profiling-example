from locust import HttpUser, task, between

class Basic(HttpUser):
    wait_time = between(0.5, 0.5)

    @task
    def cpu(self):
        self.client.get("/cpu")

    # @task
    # def cpu(self):
    #     self.client.get("/cpualloc")

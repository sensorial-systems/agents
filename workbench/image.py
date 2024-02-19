import autogen
from dotenv import load_dotenv, dotenv_values

load_dotenv()

env = dotenv_values(".env")

config_list = [
    {
        'model': env["MODEL"],
        'api_key': env["OPENAI_KEY"],
    }
]

llm_config = { "config_list": config_list }

coder = autogen.AssistantAgent("Coder", llm_config=llm_config)
executor = autogen.UserProxyAgent("Executor", human_input_mode="NEVER", is_termination_msg=lambda x: x.get("content", "").rstrip().endswith("TERMINATE"), code_execution_config={"work_dir": "coding", "use_docker": False})
executor.initiate_chat(coder, message="Show me a picture of a cat.")

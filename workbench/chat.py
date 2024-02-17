import autogen
from dotenv import load_dotenv, dotenv_values

load_dotenv()

env = dotenv_values(".env")

config_list = [
    {
        'model': env["MODEL"],
        'api_key': env["API_KEY"],
    }
]

llm_config = {
    "config_list": config_list,
    "temperature": 0.0,
    "cache_seed": 0
}

joseph = autogen.AssistantAgent("Joseph", system_message="You are a person called Joseph. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE if both of you have presented yourselves.", is_termination_msg=lambda x: "TERMINATE" in x.get("content", ""), llm_config=llm_config)
maria = autogen.AssistantAgent("Maria", system_message="You are a person called Maria. You will present yourself and you will ask the other part to present themselves. You will not present yourself as an AI model. You will say TERMINATE if both of you have presented yourselves.", is_termination_msg=lambda x: "TERMINATE" in x.get("content", ""), llm_config=llm_config)
joseph.initiate_chat(maria, message="Hi! What's your name?")

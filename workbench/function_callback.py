import autogen
from typing import Literal
from pydantic import BaseModel, Field
from typing_extensions import Annotated
from dotenv import load_dotenv, dotenv_values

load_dotenv()

env = dotenv_values(".env")

config_list = [
    {
        'model': env["MODEL"],
        'api_key': env["OPENAI_KEY"],
    }
]

llm_config = {
    "config_list": config_list,
    "temperature": 0.0,
    "cache_seed": 0
}

CurrencySymbol = Literal["USD", "EUR"]


def exchange_rate(base_currency: CurrencySymbol, quote_currency: CurrencySymbol) -> float:
    if base_currency == quote_currency:
        return 1.0
    elif base_currency == "USD" and quote_currency == "EUR":
        return 1 / 1.1
    elif base_currency == "EUR" and quote_currency == "USD":
        return 1.1
    else:
        raise ValueError(f"Unknown currencies {base_currency}, {quote_currency}")

llm_config = { "config_list": config_list }

coder = autogen.AssistantAgent("Coder", llm_config=llm_config)
executor = autogen.UserProxyAgent("Executor", human_input_mode="NEVER", is_termination_msg=lambda x: x.get("content", "") and x.get("content", "").rstrip().endswith("TERMINATE"), code_execution_config={"work_dir": "coding", "use_docker": False})

CurrencySymbol = Literal["USD", "EUR"]


def exchange_rate(base_currency: CurrencySymbol, quote_currency: CurrencySymbol) -> float:
    if base_currency == quote_currency:
        return 1.0
    elif base_currency == "USD" and quote_currency == "EUR":
        return 1 / 1.1
    elif base_currency == "EUR" and quote_currency == "USD":
        return 1.1
    else:
        raise ValueError(f"Unknown currencies {base_currency}, {quote_currency}")

@executor.register_for_execution()
@coder.register_for_llm(description="Currency exchange calculator.")
def currency_calculator(
    base_amount: Annotated[float, "Amount of currency in base_currency"],
    base_currency: Annotated[CurrencySymbol, "Base currency"] = "USD",
    quote_currency: Annotated[CurrencySymbol, "Quote currency"] = "EUR",
) -> str:
    quote_amount = exchange_rate(base_currency, quote_currency) * base_amount
    return f"{quote_amount} {quote_currency}"

executor.initiate_chat(coder, message="How much is 5 USD in EUR?")

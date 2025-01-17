import streamlit as st
from transformers import TFAutoModelForSeq2SeqLM, AutoTokenizer

# Load the translation models
translator_en_to_zh = TFAutoModelForSeq2SeqLM.from_pretrained("Helsinki-NLP/opus-mt-en-zh")
translator_zh_to_en = TFAutoModelForSeq2SeqLM.from_pretrained("Helsinki-NLP/opus-mt-zh-en")
tokenizer_en_to_zh = AutoTokenizer.from_pretrained("Helsinki-NLP/opus-mt-en-zh")
tokenizer_zh_to_en = AutoTokenizer.from_pretrained("Helsinki-NLP/opus-mt-zh-en")

st.set_page_config(
    page_title="Chinese-English Translation",
    page_icon="🌐",
    layout="wide",
    initial_sidebar_state="auto"
)

st.title('Chinese-English Translation')

def translate_en_to_zh(text):
    input_ids = tokenizer_en_to_zh.encode(text, return_tensors="tf")
    translated_ids = translator_en_to_zh.generate(input_ids)
    translated_text = tokenizer_en_to_zh.decode(translated_ids[0], skip_special_tokens=True)
    return translated_text

def translate_zh_to_en(text):
    input_ids = tokenizer_zh_to_en.encode(text, return_tensors="tf")
    translated_ids = translator_zh_to_en.generate(input_ids)
    translated_text = tokenizer_zh_to_en.decode(translated_ids[0], skip_special_tokens=True)
    return translated_text

def main():
    left_column, right_column = st.columns(2)

    header_html = """
    <div style="background-color: #264653; padding: 20px; border-radius: 10px;">
        <h1 style="color: white; text-align: center; font-size: 36px;">🌐 Chinese-English Translation</h1>
    </div>
    <br />
    """
    st.markdown(header_html, unsafe_allow_html=True)

    with left_column:
        st.subheader("Input Chinese to translate into English")
        chinese_input = st.text_area("Please enter Chinese text:", height=200)
        if st.button("Translate into English", key="translate_chinese_to_english"):
            if chinese_input:
                translated_text = translate_zh_to_en(chinese_input)
                st.markdown("<hr>", unsafe_allow_html=True)
                st.markdown("<h3>Translation Result:</h3>", unsafe_allow_html=True)
                st.write(translated_text)
                st.markdown("<hr>", unsafe_allow_html=True)
            else:
                st.warning("Please enter some Chinese text.")

    with right_column:
        st.subheader("Input English to translate into Chinese")
        english_input = st.text_area("Please enter English text:", height=200)
        if st.button("Translate into Chinese", key="translate_english_to_chinese"):
            if english_input:
                translated_text = translate_en_to_zh(english_input)
                st.markdown("<hr>", unsafe_allow_html=True)
                st.markdown("<h3>Translate Result:</h3>", unsafe_allow_html=True)
                st.write(translated_text)
                st.markdown("<hr>", unsafe_allow_html=True)
            else:
                st.warning("Please enter some English text.")

if __name__ == "__main__":
    main()

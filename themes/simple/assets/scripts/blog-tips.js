function tips_show(style, message) {
    let tips = document.querySelector(".tips");
    let tips_item = document.createElement('span');

    tips_item.innerText = message;
    tips_item.classList.add('tips-item');
    tips_item.classList.add(`${style}`);
    tips_item.classList.add('show');

    tips.appendChild(tips_item);

    TIPS_TIMEOUT_ID = setTimeout(() => { tips_close(tips_item); }, 2000);
}

function tips_close(tips_item) {
    tips_item.classList.add('hide');
    tips_item.classList.remove('show');

    setTimeout(() => { tips_item.remove(); }, 300);
}
(function () {
    modal_init();
})();

function modal_init() {
    let elements = document.querySelectorAll(".modal-open");
    elements.forEach((element) => {
        element.addEventListener("click", (event) => {
            let target = event.target.dataset.modalTarget;
            let value = event.target.dataset.modalValue;
            modal_open(target, value);
        });
    });
    let modals = document.querySelectorAll(".modal");
    modals.forEach((modal) => {
        let close = modal.querySelector(".close");
        if (close) {
            close.addEventListener("click", () => {
                modal_close(modal);
            });
        }
    });
    document.addEventListener("click", (event) => {
        modals.forEach((modal) => {
            if (event.target === modal && modal.is_mousedown) {
                modal_close(modal);
            }
            modal.is_mousedown = false;
        });
    });
    document.addEventListener("mousedown", (event) => {
        modals.forEach((modal) => {
            if (event.target === modal) {
                modal.is_mousedown = true;
            }
        });
    });
}

function modal_open(target, value) {
    let modals = document.querySelectorAll(target);
    modals.forEach((modal) => {
        let event = new CustomEvent("modal.open", {
            detail: { value: value },
        });
        modal.dispatchEvent(event);
        modal.classList.add('show');
        modal.classList.remove('hide');
        modal.style.display = "block";
    });
}

function modal_close(modal) {
    modal.classList.add('hide');
    modal.classList.remove('show');
    setTimeout(() => { modal.style.display = "none"; }, 300);
}

function modal_register(target, callback) {
    let modals = document.querySelectorAll(target);
    modals.forEach((modal) => {
        modal.addEventListener("modal.open", (event) => {
            modal.setAttribute("data-modal-value", event.detail.value);
            callback.on_open && callback.on_open(modal, modal.dataset.modalValue);
        });
        let confirm = modal.querySelector(".confirm");
        if (confirm) {
            confirm.addEventListener("click", () => {
                callback.on_confirm && callback.on_confirm(modal, modal.dataset.modalValue);
            });
        }
    });
}
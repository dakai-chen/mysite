(function () {
    pagination_init();
})();

function pagination_init() {
    let elements = document.querySelectorAll(".pagination a");
    elements.forEach((element) => {
        element.addEventListener("click", pagination_goto);
    });
}

function pagination_goto(event) {
    let search = new URLSearchParams(window.location.search);
    search.set("page", event.target.dataset.p);
    window.location.search = search.toString();
}

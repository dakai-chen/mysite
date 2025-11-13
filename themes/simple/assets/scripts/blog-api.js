function api_delete_article(id, callback) {
    let params = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ "article_id": id }),
    };
    fetch(`/api/article/remove`, params)
        .then((response) => {
            alert_error(response) && callback(response);
        })
        .catch((error) => {
            tips_show("tips-item-error", error);
        });
}

function api_delete_attachment(data, callback) {
    let params = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: data,
    };
    fetch(`/api/article/remove_attachment`, params)
        .then((response) => {
            alert_error(response) && callback(response);
        })
        .catch((error) => {
            tips_show("tips-item-error", error);
        });
}

async function api_upload_resource(name, file, callback) {
    const file_sha256 = await sha256(file);

    const headers = new Headers();
    headers.append('x-file-size', file.size.toString());
    headers.append('x-file-name', encodeURIComponent(name));
    headers.append('x-file-mime-type', file.type || 'application/octet-stream');
    headers.append('x-file-sha256', file_sha256);

    let params = {
        method: "POST",
        headers: headers,
        body: file,
    };
    fetch(`/api/resource/upload`, params)
        .then((response) => {
            alert_error(response) && callback(response);
        })
        .catch((error) => {
            tips_show("tips-item-error", error);
        });
}

async function api_upload_attachment(article_id, name, file, callback) {
    const file_sha256 = await sha256(file);

    const headers = new Headers();
    headers.append('x-article-id', article_id);
    headers.append('x-file-size', file.size.toString());
    headers.append('x-file-name', encodeURIComponent(name));
    headers.append('x-file-mime-type', file.type || 'application/octet-stream');
    headers.append('x-file-sha256', file_sha256);

    let params = {
        method: "POST",
        headers: headers,
        body: file,
    };
    fetch(`/api/article/upload_attachment`, params)
        .then((response) => {
            alert_error(response) && callback(response);
        })
        .catch((error) => {
            tips_show("tips-item-error", error);
        });
}

function alert_error(response) {
    if (response.status >= 400 && response.status <= 599) {
        response.json().then((data) => {
            tips_show("tips-item-error", data.message);
        });
        return false;
    }
    return true;
}
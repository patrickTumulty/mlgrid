

import $ from "jquery";

export default class RESTClient
{
    constructor(remote) {
        this.remote = remote;
    }

    post(url, json) {
        return $.ajax({
            type: "POST",
            url: `${this.remote}${url}`,
            cache: false,
            data: JSON.stringify(json),
            dataType: "json",
            contentType: "application/json;charset=UTF-8",
            async: false,
            success: function (data) {
                // TODO consider adding something here
            }
        });
    }

    get(url) {
        return $.ajax({
            type: "GET",
            url: `${this.remote}${url}`,
            cache: false,
            async: false,
            success: function (data) {
                // TODO consider adding something here
            }
        });
    }
}

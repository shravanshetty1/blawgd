
function captcha(response) {
    let data = {
        address: JSON.parse(localStorage.getItem("app_data")).address,
        captcha: response,
    }
    let dst = location.protocol+"//faucet."+location.hostname+":"+location.port
    console.log(data)
    console.log(dst)
    fetch(dst, {
        method: "POST",
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    }).then(res => {
        if (res.status!==200) {
            console.log(res.status)
        } else {
            window.location.href = window.location.protocol+"//"+window.location.host
        }
    });
}
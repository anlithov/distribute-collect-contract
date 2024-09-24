use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

pub fn routes_root() -> Router {
    Router::new().route("/", get(ui))
}

async fn ui() -> impl IntoResponse {
    println!("->> {:<12} - home", "HANDLER");

    let html_content = r#"
    <html>
    <head>
        <title>Distribute Tokens</title>
    </head>
    <body>
        <h1>Distribute Native Tokens</h1>
    <form id="nativeForm" action="/distribute/native" method="POST">
        <label for="receivers">Receivers (comma separated)</label>
        <input type="text" id="receivers" name="receivers" placeholder="Enter receiver addresses, e.g., 0xabc,0xdef">

        <label for="proportions">Proportions (comma separated)</label>
        <input type="text" id="proportions" name="proportions" placeholder="Enter proportions, e.g., 50,50">

        <label for="amount">Amount</label>
        <input type="number" id="amount" name="amount" placeholder="Enter the total amount">

        <button type="button" onclick="submitNative()">Submit Native Token Distribution</button>
    </form>

    <h1>Distribute ERC20 Tokens</h1>
    <form id="erc20Form" action="/distribute/erc20" method="POST">
        <label for="tokenAddress">ERC20 Token Address</label>
        <input type="text" id="tokenAddress" name="token_address" placeholder="Enter the ERC20 token contract address">

        <label for="erc20Receivers">Receivers (comma separated)</label>
        <input type="text" id="erc20Receivers" name="receivers" placeholder="Enter receiver addresses, e.g., 0xabc,0xdef">

        <label for="erc20Proportions">Proportions (comma separated)</label>
        <input type="text" id="erc20Proportions" name="proportions" placeholder="Enter proportions, e.g., 50,50">

        <label for="erc20Amount">Amount</label>
        <input type="number" id="erc20Amount" name="amount" placeholder="Enter the total amount">

        <button type="button" onclick="submitERC20()">Submit ERC20 Token Distribution</button>
    </form>

    <h1>Collect ERC20 Tokens</h1>
    <h3>Warning! Approve required from those we collect token from.</h3>
    <form id="collectErc20Form" action="/collect/erc20" method="POST">
        <label for="tokenAddress">ERC20 Token Address</label>
        <input type="text" id="tokenAddress" name="token_address" placeholder="Enter the ERC20 token contract address">

        <label for="erc20Receivers">Collect from wallets (comma separated)</label>
        <input type="text" id="erc20Receivers" name="froms" placeholder="Enter receiver addresses, e.g., 0xabc,0xdef">

        <label for="erc20Proportions">SCALED Percents of each wallet (multiplied by 1_000_000) (comma separated)</label>
        <input type="text" id="erc20Proportions" name="percents" placeholder="Enter scaled percents, e.g., if 50.657444 then write 50657444">

        <button type="button" onclick="submitCollectERC20()">Submit ERC20 Token Collection</button>
    </form>

    <style>
      form {
         display: flex;
         flex-direction: column;
         width: 400px;
      }
    </style>

    <script>
        // Helper function to convert comma-separated strings into JSON array format
        function createReceiversWithProportions(receivers, proportions) {
            let receiversArray = receivers.split(',').map(item => item.trim());
            let proportionsArray = proportions.split(',').map(item => item.trim());

            return receiversArray.map((receiver, index) => ({
                receiver: receiver,
                proportion: proportionsArray[index] || '0'
            }));
        }

        // Submit form data for native token distribution
        function submitNative() {
            const form = document.getElementById('nativeForm');
            const receivers = form.receivers.value;
            const proportions = form.proportions.value;
            const amount = String(form.amount.value);

            const payload = {
                receivers_with_proportions: createReceiversWithProportions(receivers, proportions),
                amount
            };

            fetch(form.action, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(payload)
            })
            .then(response => response.json())
            .then(data => alert('Response: ' + JSON.stringify(data)))
            .catch(error => console.error('Error:', error));
        }

        // Submit form data for ERC20 token distribution
        function submitERC20() {
            const form = document.getElementById('erc20Form');
            const receivers = form.receivers.value;
            const proportions = form.proportions.value;
            const amount = String(form.amount.value);
            const tokenAddress = form.token_address.value;

            const payload = {
                base: {
                    receivers_with_proportions: createReceiversWithProportions(receivers, proportions),
                    amount
                },
                token_address: tokenAddress
            };

            fetch(form.action, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(payload)
            })
            .then(response => response.json())
            .then(data => alert('Response: ' + JSON.stringify(data)))
            .catch(error => console.error('Error:', error));
        }


        function createSets(froms, percents) {
            let fromsArray = froms.split(',').map(item => item.trim());
            let percentsArray = percents.split(',').map(item => item.trim());

            return fromsArray.map((from, index) => ({
                from,
                scaled_percent: percentsArray[index] || '0'
            }));
        }

        // Submit form data for ERC20 token distribution
        function submitCollectERC20() {
            const form = document.getElementById('collectErc20Form');
            const froms = form.froms.value;
            const percents = form.percents.value;
            const tokenAddress = form.token_address.value;

            const payload = {
                sets: createSets(froms, percents),
                token_address: tokenAddress
            };

            fetch(form.action, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(payload)
            })
            .then(response => response.json())
            .then(data => alert('Response: ' + JSON.stringify(data)))
            .catch(error => console.error('Error:', error));
        }
    </script>
    </body>
    </html>
    "#;

    Html(html_content)
}

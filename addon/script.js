
(async function() {

    if(document.body.getAttribute('data-page_name') != 'view_game') return;

    const gameInfo = await (await fetch(`${location.origin}${location.pathname}/data.json`)).json();
    console.log(gameInfo);

    if(typeof gameInfo != 'object' || gameInfo == null) return;
    if(!('id' in gameInfo) || typeof gameInfo.id != 'number') return;

    const gameId = gameInfo.id;

    const buttonsList = document.querySelector('#user_tools');

    buttonsList.insertAdjacentHTML('beforeend', `
        <li>
            <a class="action_btn" href="itch-io-downloader://${gameId}">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-download"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
                Download & Play
            </a>
        </li>
    `);

})();

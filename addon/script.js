
(async function() {

    if(document.body.getAttribute('data-page_name') != 'view_game') return;

    const gameInfo = await (await fetch(`${location.origin}${location.pathname}/data.json`)).json();
    console.log(gameInfo);

    if(typeof gameInfo != 'object' || gameInfo == null) return;
    if(!('id' in gameInfo) || typeof gameInfo.id != 'number') return;

    const gameId = gameInfo.id;

    const buttonsList = document.querySelector('#user_tools');
    const button = document.createElement('li');
    buttonsList.appendChild(button);
    const link = document.createElement('a');
    button.appendChild(link);
    link.classList.add('action_btn');
    link.href = `itch-io-downloader://${gameId}`;
    link.innerText = 'Download & play this game'

})();

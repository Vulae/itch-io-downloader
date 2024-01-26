


/**
 * @typedef {object} GameInfo
 * @property {number} id
 */

/**
 * @param {unknown} value
 * @returns {GameInfo | null}
 */
function validateGameInfo(value) {
    if(typeof value != 'object' || value == null) return null;
    if(!('id' in value) || typeof value.id != 'number') return null;
    return value;
}

/**
 * @returns {Promise<GameInfo | null>}
 */
async function getGameInfo() {
    const gameInfo = await (await fetch(`${location.origin}${location.pathname}/data.json`)).json();
    console.log(gameInfo);
    return validateGameInfo(gameInfo);
}



/**
 * @param {HTMLDivElement} element #user_tools 
 * @param {GameInfo} gameInfo 
 */
async function displayPlayButton(element, gameInfo) {

    element.insertAdjacentHTML('beforeend', `
        <li>
            <a class="action_btn" href="itch-io-downloader://play/${gameInfo.id}">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-download"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
                Download & Play
            </a>
        </li>
    `);

}



/**
 * @param {HTMLDivElement} element .primary_header
 */
function displayLauncherButton(element) {

    const headerButtons = element.querySelector('.header_buttons');

    headerButtons.insertAdjacentHTML('beforeend', `
        <a class="header_button" href="itch-io-downloader://play">Launcher</a>
    `);

}



async function displayPage() {

    document.querySelectorAll('.primary_header').forEach(element => {
        displayLauncherButton(element);
    });



    if(document.body.getAttribute('data-page_name') == 'view_game') {
        const gameInfo = await getGameInfo();

        if(!gameInfo) {
            throw new Error('Failed to get game info.');
        }

        document.querySelectorAll("#user_tools").forEach(element => {
            displayPlayButton(element, gameInfo);
        });
    }

}



displayPage();

const loadMap = () => {
    return new jsVectorMap({
        selector: '#map',
        map: 'world',
        regionStyle: {
            selected: {
                fill: 'red',
            },

        },
        onRegionClick: function (_, code) {
            const name = encodeURIComponent(this.regions[code]?.config.name || "");
            htmx.ajax(
                'GET',
                `/countries/${code}?name=${name}`,
                { target: '#country-details-dialog-content', swap: 'innerHTML' },
            ).then(
                () => {
                    const dialog = document.getElementById('country-details-dialog');

                    if (!dialog) {
                        console.error('Unable to locate dialog');
                        return;
                    }

                    dialog.showModal();
                }
            ).catch((error) => console.error(error));
        }
    });
}

const loadCountries = () => {
    const countriesData = document.getElementById('countries-list');

    if (!countriesData) {
        console.error('Countries list element not found');
        return null;
    }

    try {
        return JSON.parse(countriesData.textContent);
    } catch (error) {
        console.error('Error parsing countries data:', error);
        return null;
    }
}

const showCountries = (map, countriesData) => {
    if (!countriesData) {
        return;
    }

    map.setSelectedRegions(countriesData.map(country => country.code));
}

const init = () => {
    const map = loadMap();
    const countriesData = loadCountries();

    showCountries(map, countriesData);
};

init();

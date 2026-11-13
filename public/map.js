const loadMap = () => {
    return new jsVectorMap({
        selector: '#map',
        map: 'world',
        regionStyle: {
            selected: {
                fill: 'red',
            },

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

Feature: AssetPairs feature

    Scenario: Retrieve all information from an asset pair
        When a User make an http request to "/0/public/AssetPairs?pair=<pair1>,<pair2>"
        Then http response status code is "200"
        And the AssetPairs response body contains a valid asset pair response format
        And the response body does not contain any error

        Examples:
            |  pair1  |  pair2  |
            |  xbtusd |  xbteur |

    Scenario: Retrieve fees information from an asset pair
        When a User make an http request to "/0/public/AssetPairs?pair=<pair1>,<pair2>&info=fees"
        Then http response status code is "200"
        And the AssetPairs response body contains a valid asset pair response format with fees
        And the response body does not contain any error

        Examples:
            |  pair1  |  pair2  |
            |  xbtusd |  xbteur |

    Scenario: Retrieve margin information from an asset pair
        When a User make an http request to "/0/public/AssetPairs?pair=<pair1>,<pair2>&info=margin"
        Then http response status code is "200"
        And the AssetPairs response body contains a valid asset pair response format with margin
        And the response body does not contain any error

        Examples:
            |  pair1  |  pair2  |
            |  xbtusd |  xbteur |


Feature: AssetPairs feature

    Scenario: Retrieve all information from an asset pair
        When User make GET http request to "/0/public/AssetPairs?pair=<asset1><asset2>"
        Then http response status code is "200"
        And the AssetPairs response body contains a valid asset pair response format
        And the AssetPairs response body does not contain any error

        Examples:
            | asset1 | asset2 |
            |    xbt |    usd |
            |    xbt |    eur |

    Scenario: Retrieve fees information from an asset pair
        When User make GET http request to "/0/public/AssetPairs?pair=<asset1><asset2>&info=fees"
        Then http response status code is "200"
        And the AssetPairs response body contains fees information
        And the AssetPairs response body does not contain any error

        Examples:
            | asset1 | asset2 |
            |    xbt |    usd |
            |    xbt |    eur |


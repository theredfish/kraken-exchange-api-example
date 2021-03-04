Feature: OpenOrders feature

    Scenario: Retrieve open orders
        Given a User with an API key and 2FA configured
        When a User make an http request to "/0/private/OpenOrders"
        Then http response status code is "200"
        And the response contains a valid OpenOrders result
        And the response body does not contain any error

Feature: Time feature

    Scenario: Retrieve the server time
        When a User make an http request to "/0/public/Time"
        Then http response status code is "200"
        And the Time response body contains a valid response format
        And the response body does not contain any error

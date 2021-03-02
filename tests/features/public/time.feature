Feature: Time feature

    Scenario: Retrieve the server time
        When User make GET http request to "/0/public/Time"
        Then http response status code is "200"
        And the Time response body contains a valid response format
        And the Time response body does not contain any error

    Scenario: Example of a not implemented scenario to show in logs
        Given a not yet implemented scenario
        When the framework reach the given statement
        Then the framework outputs "Not yet implemented (skipped)"
        And it is useful when stakeholders (for example a product owner) want to write scenarios without implementation

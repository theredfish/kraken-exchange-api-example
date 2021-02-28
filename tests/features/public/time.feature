Feature: Time feature

    Scenario: Retrieve the server time
        When I access the server time from /0/public/Time
        Then the http status code should be 200
        And the response body contains a valid response format
        And the response body does not contain any error

    Scenario: Example of a not implemented scenario to show in logs
        Given a not yet implemented scenario
        When the framework reach the given statement
        Then the framework outputs "Not yet implemented (skipped)"
        And it is useful when stakeholders (for example a product owner) want to write scenarios without implementation

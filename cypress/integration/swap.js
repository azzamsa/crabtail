describe("Swap Test", () => {
  it("Visits Crabtail", () => {
    cy.visit("/")
    //cy.pause()

    // prepare state
    cy.fixture("base").then((base) => {
      cy.get("#input").within(() => {
        cy.get("textarea").type(base.css.value)
      })
      cy.contains("Go").click()
    })

    // check state after swap
    cy.fixture("base").then((base) => {
      cy.get("#swap-btn").click()

      cy.get("#input").within(() => {
        // now swapped
        cy.get("label").contains(base.typed.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.typed.placeholder)
          .should("have.attr", "value", base.typed.value)
      })

      cy.get("#output").within(() => {
        cy.get("label").contains(base.css.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.css.placeholder)
          .should("have.attr", "value", base.css.value)
      })
    })

    // swap again
    cy.fixture("base").then((base) => {
      cy.get("#swap-btn").click()

      cy.get("#input").within(() => {
        // now swapped
        cy.get("label").contains(base.css.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.css.placeholder)
          .should("have.attr", "value", base.css.value)
      })

      cy.get("#output").within(() => {
        cy.get("label").contains(base.typed.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.typed.placeholder)
          .should("have.attr", "value", base.typed.value)
      })
    })
  })
})

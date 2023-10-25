import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://ecommerce-playground.lambdatest.io/index.php?route=account/login')

WebUI.click(findTestObject('Object Repository/Page_Account Login/div_New Customer Register Account By creati_d2cb4b'))

WebUI.click(findTestObject('Object Repository/Page_Account Login/a_Continue'))

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_First Name_firstname'), 'A')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Last Name_lastname'), 'G')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_E-Mail_email'), '221123@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Register Account/input_Telephone_telephone'), '1122334455')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password_password'), 'Z/VL8UOjsxo=')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Register Account/input_Password Confirm_confirm'), 'Z/VL8UOjsxo=')

WebUI.click(findTestObject('Object Repository/Page_Register Account/label_I have read and agree to the Privacy Policy'))

WebUI.click(findTestObject('Object Repository/Page_Register Account/input_Privacy Policy_btn btn-primary'))


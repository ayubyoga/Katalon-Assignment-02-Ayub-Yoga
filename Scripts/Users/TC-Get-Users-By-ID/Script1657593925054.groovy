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

response6 = WS.sendRequest(findTestObject('Users/GET - Users By ID'))

assert response6.getStatusCode() == 200

WS.verifyElementPropertyValue(response6, 'id', '9')
WS.verifyElementPropertyValue(response6, 'name', 'Glenna Reichert')
WS.verifyElementPropertyValue(response6, 'username', 'Delphine')
WS.verifyElementPropertyValue(response6, 'email', 'Chaim_McDermott@dana.io')
WS.verifyElementPropertyValue(response6, 'address.street', 'Dayna Park')
WS.verifyElementPropertyValue(response6, 'address.suite', 'Suite 449')
WS.verifyElementPropertyValue(response6, 'address.city', 'Bartholomebury')
WS.verifyElementPropertyValue(response6, 'address.zipcode', '76495-3109')
WS.verifyElementPropertyValue(response6, 'address.geo.lat', '24.6463')
WS.verifyElementPropertyValue(response6, 'address.geo.lng', '-168.8889')
WS.verifyElementPropertyValue(response6, 'phone', '(775)976-6794 x41206')
WS.verifyElementPropertyValue(response6, 'website', 'conrad.com')
WS.verifyElementPropertyValue(response6, 'company.name', 'Yost and Sons')
WS.verifyElementPropertyValue(response6, 'company.catchPhrase', 'Switchable contextually-based project')
WS.verifyElementPropertyValue(response6, 'company.bs', 'aggregate real-time technologies')
